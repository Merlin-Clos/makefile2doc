# ==============================================================================
## @category Development
# ==============================================================================

## @description Start the full development environment (Docker)
## @env PORT
up:
	# docker compose up -d

## @description Stop all containers
down:
	# docker compose down

## @description Show live logs for all services
## @depends up
logs:
	# docker compose logs -f

## @description Open a shell inside the PHP container (Laravel)
## @depends up
shell-back:
	# docker compose exec app bash

## @description Open a shell inside the Node.js container
## @depends up
shell-front:
	# docker compose exec app bash

## @description Open a shell inside the PostgreSQL container
## @depends up
shell-db:
	# docker compose exec app bash

# ==============================================================================
## @category Setup
# ==============================================================================

## @description Install backend (Composer)
install-back:
	# docker compose run --rm app composer install

## @description Install frontend (NPM) dependencies
install-front:
	# docker compose run --rm app npm install

## @description Run both backend and frontend installations
## @depends install-back, install-front
install: install-back install-front

# ==============================================================================
## @category Database
# ==============================================================================

## @description Run database migrations
## @depends up, install
migrate:
	# docker compose exec app php artisan migrate

## @description Reset the DB and run seeds (Test data) \n Warning: This deletes all data!
## @depends migrate
## @env SEED_CLASS
seed:
	# docker compose exec app php artisan migrate:refresh --seed

# ==============================================================================
## @category Quality
# ==============================================================================

## @description Run unit tests (Pest/PHPUnit)
## @depends install
test-back:
	# docker-compose exec app php artisan test

## @description Lint frontend code (ESLint)
## @depends install-front
lint-front:
	# docker compose exec app npm run lint

## @description Lint backend code (PHP-CS-Fixer dry-run)
## @depends install-back
lint-back:
	# docker compose exec app php-cs-fixer

## @description Fix frontend code style and format
## @depends install-front
fix-front:
	# docker compose exec app npm run lint-fix && npm run format

## @description Fix backend code style
## @depends install-back
fix-back:
	# docker compose exec app php-cs-fixer --fix

## @description Run all linters (front & back)
## @depends lint-front, lint-back
lint: lint-front lint-back

## @description Fix all code style issues (front & back)
## @depends fix-front, fix-back
fix: fix-front fix-back

# ==============================================================================
## @category Deployment
# ==============================================================================

## @description Compile Frontend assets (Vite/Mix)
## @depends install
build-front:
	# npm run build

## @description Deploy to Production \n 1. Build frontend assets \n 2. Optimize Laravel cache \n 3. Run migrations force
## @depends build-front, migrate
## @env APP_KEY, SSH_USER
deploy:
	# git pull
	# docker compose exec app php artisan config:cache
	# docker compose exec app php artisan migrate --force