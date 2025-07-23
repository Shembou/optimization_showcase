FROM node:latest AS builder

ARG PORT=443
ENV PORT=${PORT}

WORKDIR /app
COPY package*.json ./
RUN npm ci --only=production
COPY . .
RUN npm run build

FROM nginx:alpine
COPY --from=builder /app/dist/ /usr/share/nginx/html
COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh
COPY nginx.conf.template /etc/nginx/nginx.conf.template
COPY certs/localhost/cert.pem /etc/nginx/certs/cert.pem
COPY certs/localhost/key.pem /etc/nginx/certs/key.pem


EXPOSE ${PORT}

ENTRYPOINT [ "/entrypoint.sh" ]