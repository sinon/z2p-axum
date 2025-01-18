prog :=xnixperms

debug ?=

$(info debug is $(debug))

ifdef debug
  release :=
  target :=debug
  extension :=debug
else
  release :=--release
  target :=release
  extension :=
endif

docker_run:
	docker run -p 5432:5432 --name some-postgres -e POSTGRES_PASSWORD=password -e POSTGRES_DB=newsletter -d postgres

build:
	cargo build $(release)
migrate:
	DATABASE_URL=postgres://postgres:password@localhost:5432/newsletter cargo sqlx migrate run
test:
	DATABASE_URL=postgres://postgres:password@localhost:5432/newsletter cargo test
run:
	DATABASE_URL=postgres://postgres:password@localhost:5432/newsletter cargo run

all: build install
 
help:
	@echo "usage: make $(prog) [debug=1]"