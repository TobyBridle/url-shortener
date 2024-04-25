FROM node:latest
WORKDIR /app-frontend

COPY package*.json /app-frontend/

RUN npm install

COPY . /app-frontend/

# RUN npm run build

EXPOSE 8080
CMD [ "npx", "vite", "--host=0.0.0.0", "--port=8080" ]