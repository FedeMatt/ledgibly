---
name: database-design
description: Design database schemas — tables, relationships, indexes, constraints, and ORM setup. Use when adding or changing tables, FKs, indexes, or Prisma/Drizzle models.
---

# Database Design

Design a database schema from requirements.

## Workflow

### 1. Identify Entities

From the requirements, extract the core entities (nouns):
- Users, Teams, Projects, Tasks, Comments, etc.
- Each entity becomes a table

### 2. Define Relationships

| Relationship | Implementation |
|-------------|----------------|
| One-to-one | Foreign key with unique constraint, or embed in same table |
| One-to-many | Foreign key on the "many" side |
| Many-to-many | Junction/join table |
| Self-referential | Foreign key pointing to same table (e.g. `parent_id`) |

### 3. Design the Schema

For each table:

```sql
CREATE TABLE users (
  id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  email       TEXT NOT NULL UNIQUE,
  name        TEXT NOT NULL,
  avatar_url  TEXT,
  created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE projects (
  id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name        TEXT NOT NULL,
  owner_id    UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);
```

### 4. Apply Best Practices

**Primary keys:**
- Use `UUID` for distributed systems or public-facing IDs
- Use `SERIAL`/`BIGSERIAL` for internal-only IDs (faster joins)

**Timestamps:**
- Always add `created_at` and `updated_at`
- Use `TIMESTAMPTZ` (with timezone), never `TIMESTAMP`

**Naming:**
- Tables: plural snake_case (`users`, `project_members`)
- Columns: singular snake_case (`user_id`, `created_at`)
- Indexes: `idx_<table>_<columns>` (`idx_users_email`)

**Constraints:**
- `NOT NULL` on everything unless it is genuinely optional
- `UNIQUE` on natural keys (email, slug, external IDs)
- `REFERENCES` with `ON DELETE` behavior (CASCADE, SET NULL, RESTRICT)
- `CHECK` constraints for enums or value ranges

### 5. Add Indexes

```sql
CREATE INDEX idx_projects_owner_id ON projects(owner_id);
CREATE UNIQUE INDEX idx_users_email ON users(email);
CREATE INDEX idx_tasks_project_status ON tasks(project_id, status);
```

**When to index:** foreign keys, columns in WHERE / ORDER BY / JOIN.

**When not to index:** small tables, low-cardinality columns, columns rarely queried.

### 6. ORM Setup

**Prisma:**

```prisma
model User {
  id        String   @id @default(uuid())
  email     String   @unique
  name      String
  projects  Project[]
  createdAt DateTime @default(now()) @map("created_at")
  updatedAt DateTime @updatedAt @map("updated_at")
  @@map("users")
}
```

**Drizzle:**

```typescript
export const users = pgTable('users', {
  id: uuid('id').primaryKey().defaultRandom(),
  email: text('email').notNull().unique(),
  name: text('name').notNull(),
  createdAt: timestamp('created_at', { withTimezone: true }).notNull().defaultNow(),
  updatedAt: timestamp('updated_at', { withTimezone: true }).notNull().defaultNow(),
});
```

## Common Patterns

- Soft deletes: `deleted_at TIMESTAMPTZ`
- Audit log: `audit_events` with `entity_type`, `entity_id`, `action`, `actor_id`, `payload`
- Tags: junction table
- Tree: `parent_id` or materialized path
- Polymorphic associations: prefer separate FKs over `entity_type` + `entity_id`

## Tips

- Start normalized (3NF); denormalize only when you have measured a problem
- Do not store derived data without an invalidation strategy
- Use database enums or check constraints for status fields
- Always decide what happens when a parent record is deleted
