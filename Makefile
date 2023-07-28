build:
	docker compose build
up:
	docker compose up -d $(ARGS)
restart:
	docker compose stop $(ARGS)
	docker compose rm -f $(ARGS)
	docker compose up -d $(ARGS)
down:
	docker compose down --rmi all --volumes --remove-orphans
ps:
	docker compose ps
log:
	docker compose logs -f $(ARGS)
cli:
	surreal sql --conn http://localhost:8000 --user root --pass root --ns headless_sns --db headless_sns
