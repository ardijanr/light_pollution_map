FROM docker.io/node:20-bullseye

RUN apt update -y
RUN apt upgrade -y

WORKDIR /demo

COPY . .

RUN npm install

EXPOSE 3000
CMD ["npm", "start"]
