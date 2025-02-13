# dioxus-tailwind-v4

Tailwind v4 introduced changes which make most online documentation,
Q/As, and the official dioxus tailwind examples no longer correct.
These changes include:

* [CSS First Configuration](https://tailwindcss.com/blog/tailwindcss-v4#css-first-configuration).
  The `tailwind.config.js` file is no longer used or necessary. Notably the
  `content` config variable isn't needed due to
  [automatic content detection](https://tailwindcss.com/blog/tailwindcss-v4#automatic-content-detection).
* The CLI has been split from [`tailwindcss`](https://www.npmjs.com/package/tailwindcss)
  to a separate [`@tailwindcss/cli`](https://www.npmjs.com/package/@tailwindcss/cli)
  npm package. The `npx tailwindcss init` command is no longer available (or
  necessary, see above).
  
This repo attempts to provide a working example of the dioxus starter app
styled using tailwind v4.

CSS from the starter app was translated to tailwind classes manually; it may not
look exactly the same. If you notice any significant differences feel free to
send a PR.

### Organization

The organization of this repo is a hybrid of the dioxus "Jumpstart" and
"Workspace" templates, with fullstack enabled. It contains two crates, one for
the server and one for the client app. The app crate supports all
web/desktop/mobile platforms. 

```
├── app
│   ├── Cargo.toml
│   ├── assets
│   │   └── tailwind.css
│   ├── input.css
│   └── src
│       ├── components
│       ├── views
│       └── main.rs
├── server
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── Cargo.toml
├── Dioxus.toml
└── package.json
```

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

