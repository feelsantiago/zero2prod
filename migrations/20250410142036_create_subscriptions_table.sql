-- Create Subscription Table

CREATE TABLE subscriptions (
  id uuid NOT NULL,
  PRIMARY KEY (id),
  email text UNIQUE NOT NULL,
  name text NOT NULL,
  subscribed_at timestamptz NOT NULL DEFAULT (now())
);
