include .env
export

DATABASE_URL := mysql://${DB_USER}:${DB_PASS}@${DB_HOST}:${DB_PORT}/${DB_NAME}


start:
	@cargo run
	# @cargo watch -x run

up:
	@docker-compose up -d

down:
	@docker-compose down

clean:
	@rm -rf tmp/
