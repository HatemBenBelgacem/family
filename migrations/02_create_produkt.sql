
CREATE TABLE IF NOT EXISTS produkt (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    bezeichnung TEXT NOT NULL,
    eingekauft BOOLEAN NOT NULL
);