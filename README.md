# dioxus-tailwind-v4

Tailwind v4 introduced two major changes, such that most online documentation,
Q/As, and the dioxus tailwind examples are no longer correct:

* [CSS First Configuration](https://tailwindcss.com/blog/tailwindcss-v4#css-first-configuration) -
  the `tailwind.config.js` file is no longer used or necessary; notably the
  `content` config variable isn't needed due to
  [automatic content detection](https://tailwindcss.com/blog/tailwindcss-v4#automatic-content-detection).
* CLI split from [`tailwindcss`](https://www.npmjs.com/package/tailwindcss)
  to a separate [`@tailwindcss/cli`](https://www.npmjs.com/package/@tailwindcss/cli)
  npm package. Some commands, such as `npx tailwindcss init` are no longer
  available.
  
This repo attempts to provide a working example of the dioxus starter app
styled using tailwind. CSS from the starter app was translated to tailwind
classes manually, so it may not look exactly the same. If you notice any
major differences please send a PR.

### Organization

This repo is a hybrid of the dioxus "Jumpstart" and "Workspace" templates,
with fullstack enabled. It contains two crates, one for the server and one
for the client app. The app crate supports all web/desktop/mobile platforms. 

## Compiling Tailwind CSS

Install `@tailwindcss/cli`, if necessary:

```bash
# (from project root directory)
npm install
```

To run tailwind once:

```bash
npx @tailwindcss/cli -i ./app/input.css -o ./app/assets/tailwind.css
```

To have tailwind continually watch for file changes:

```bash
npx @tailwindcss/cli -i ./app/input.css -o ./app/assets/tailwind.css --watch
```

## Serving

Navigate to the app crate:

```bash
cd app
```

To serve the default platform (web):

```bash
dx serve
```

To serve a different platform:

```bash
dx serve --platform desktop
```

