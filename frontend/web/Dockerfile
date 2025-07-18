FROM node:latest AS builder
WORKDIR /app
COPY package*.json ./
RUN npm ci --only=production
COPY . .
RUN npm run build

FROM nginx:alpine
COPY --from=builder /app/dist/ /usr/share/nginx/html
COPY nginx.conf /etc/nginx/nginx.conf
COPY certs/localhost/cert.pem /etc/nginx/certs/cert.pem
COPY certs/localhost/key.pem /etc/nginx/certs/key.pem
EXPOSE 443/udp
EXPOSE 443/tcp
CMD ["nginx", "-g", "daemon off;"]