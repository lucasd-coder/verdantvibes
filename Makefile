.PHONE: infra-up
infra-up:
	docker-compose up -d

infra-down:
	docker-compose down --volumes --remove-orphans