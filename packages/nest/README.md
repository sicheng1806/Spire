# Development

## compile css file

1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the tailwind css cli: https://tailwindcss.com/docs/installation
3. Run the following command in the root of the project to start the tailwind CSS compiler:

```bash
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```

Run the following command in the root of the project to start the Dioxus dev server:

## web

```bash
dx serve --bin nest --hot-reload --port <your-port>
```

- Open the browser to http://localhost:<your-port>
- 8080 is the default port.