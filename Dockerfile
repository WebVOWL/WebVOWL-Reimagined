FROM node:22-alpine

WORKDIR /code

COPY package.json /code/.
COPY package-lock.json /code/.

RUN npm ci

COPY .cargo /code/.cargo
COPY Cargo.toml /code/.
COPY rust-toolchain.toml /code/.
COPY global-paths.js /code/.
COPY rspack.config.js /code/.
COPY tsconfig.json /code/.
COPY src /code/src
COPY LICENSE /code/.

RUN npm install serve -g
RUN npm run release

CMD ["serve", "deploy"]
