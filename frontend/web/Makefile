serve:
	npm run serve
tailwind:
	npx @tailwindcss/cli -i ./src/input.css -o ./src/output.css -m --watch
build: 
	npm run build
run: build tailwind serve 
dev: 
	@$(MAKE) run -j3
http1:
	npm run build & \
	docker build -f Dockerfile.http1 -t http1 . & \
	docker run -p 8080:80 http1
http2:
	npm run build & \
	docker build -f Dockerfile.http2 -t http2 . & \
	docker run -p 443:443 http2
http3:
	npm run build & \
	docker build -f Dockerfile.http3 -t http3 . & \
	docker run -p 443:443 http3