-- UpEnv API database schema.
-- Run this in the Supabase SQL editor.
--
-- RLS is enabled so the API uses the user's Supabase JWT through PostgREST.
-- The API deliberately does NOT require the service_role key.

create table if not exists public.devices (
    id uuid primary key default gen_random_uuid(),
    user_id uuid not null references auth.users(id) on delete cascade,
    name text not null,
    platform text not null,
    architecture text,
    app_version text,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

create table if not exists public.backups (
    id uuid primary key default gen_random_uuid(),
    user_id uuid not null references auth.users(id) on delete cascade,
    device_id uuid not null references public.devices(id) on delete cascade,
    name text not null,
    manifest jsonb not null default '{}'::jsonb,
    status text not null default 'ready'
        check (status in ('pending', 'ready', 'restoring', 'failed')),
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

create index if not exists devices_user_id_idx
    on public.devices(user_id);

create index if not exists backups_user_id_idx
    on public.backups(user_id);

create index if not exists backups_device_id_idx
    on public.backups(device_id);

alter table public.devices enable row level security;
alter table public.backups enable row level security;

create policy "Users can read their own devices"
on public.devices for select
using (auth.uid() = user_id);

create policy "Users can create their own devices"
on public.devices for insert
with check (auth.uid() = user_id);

create policy "Users can update their own devices"
on public.devices for update
using (auth.uid() = user_id)
with check (auth.uid() = user_id);

create policy "Users can delete their own devices"
on public.devices for delete
using (auth.uid() = user_id);

create policy "Users can read their own backups"
on public.backups for select
using (auth.uid() = user_id);

create policy "Users can create their own backups"
on public.backups for insert
with check (
    auth.uid() = user_id
    and exists (
        select 1 from public.devices d
        where d.id = device_id
          and d.user_id = auth.uid()
    )
);

create policy "Users can update their own backups"
on public.backups for update
using (auth.uid() = user_id)
with check (auth.uid() = user_id);

create policy "Users can delete their own backups"
on public.backups for delete
using (auth.uid() = user_id);

-- Keep updated_at current.
create or replace function public.set_updated_at()
returns trigger
language plpgsql
as $$
begin
    new.updated_at = now();
    return new;
end;
$$;

drop trigger if exists devices_set_updated_at on public.devices;
create trigger devices_set_updated_at
before update on public.devices
for each row execute function public.set_updated_at();

drop trigger if exists backups_set_updated_at on public.backups;
create trigger backups_set_updated_at
before update on public.backups
for each row execute function public.set_updated_at();
