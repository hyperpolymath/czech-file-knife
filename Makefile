.PHONY: deps kafka validate orchestrator ui run dev clean

# --- Configuration ---
NICKEL_MANIFEST = manifest/sample.ncl
VALIDATED_OUTPUT = manifest/validated.json

# --- Core Targets ---
deps:
	@echo "--- 🛠️ Checking Dependencies ---"
	bash ops/scripts/ensure_deps.sh

kafka:
	@echo "--- 🐳 Starting Infrastructure (Kafka/Postgres) ---"
	bash ops/scripts/ensure_kafka.sh

validate:
	@echo "--- 📜 Running Haskell Validator (Policy check) ---"
	# Pipe Nickel config through the Haskell CLI
	cd validator && stack run < ../$(NICKEL_MANIFEST) > ../$(VALIDATED_OUTPUT)

orchestrator:
	@echo "--- 🧪 Building & Running Elixir Orchestrator ---"
	cd orchestrator && MIX_ENV=dev mix setup
	cd orchestrator && MIX_ENV=dev mix run --no-halt

ui:
	@echo "--- 🎨 Building & Running Svelte UI ---"
	cd ui && npm install
	cd ui && npm run dev

run:
	@echo "--- 🚀 Launching Full Stack (Orchestrator + UI) ---"
	$(MAKE) orchestrator & 
	$(MAKE) ui

dev: deps kafka validate
	@echo "--- 🚀 Launching Full Dev Environment ---"
	$(MAKE) run

clean:
	@echo "--- 🧹 Cleaning ---"
	docker compose -f ops/docker-compose.yml down -v
	rm -f $(VALIDATED_OUTPUT)
	rm -rf orchestrator/_build ui/node_modules
