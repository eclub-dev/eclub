include .env

GITHOOKS_SRC = $(wildcard .githooks/*)
GITHOOKS_DEST = $(patsubst .githooks/%, .git/hooks/%, $(GITHOOKS_SRC))

.git/hooks:
	mkdir .git/hooks

.git/hooks/%: .githooks/%
	cp $^ $@

.PHONY: githooks
githooks: .git/hooks $(GITHOOKS_DEST)
	git config advice.ignoredHook false
	git config core.hooksPath .githooks


.PHONY: init
init: githooks
	bash scripts/init.sh

.PHONY: build
build: githooks
	bash scripts/build.sh

.PHONY: hmac-key
 hmac-key:
	echo -n "value" | openssl dgst -sha384 -hmac "key"

# ----------------------------------------ORM----------------------------------------
.PHONY: migrate-init
migrate-init:
	sea-orm-cli migrate init

.PHONY: migrate-refresh
migrate-refresh:
	sea-orm-cli migrate refresh

.PHONY: migrate-generate
migrate-generate:
	sea-orm-cli generate entity --with-serde both --date-time-crate time\
        -u $(DATABASE_URL) \
        -o src/domain/models
