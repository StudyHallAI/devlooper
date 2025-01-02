.PHONY: lint format test build

lint:
	ruff .
	black --check .

format:
	black .
	prettier --write .

test:
	pytest

build:
	python setup.py sdist bdist_wheel
