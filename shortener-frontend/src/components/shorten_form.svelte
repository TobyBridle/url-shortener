<script lang="ts">
  import "@fortawesome/fontawesome-free/css/all.min.css";
  import { ShortenUrl } from "../protos/shorten-url-post";
  import { ShortenedResponse } from "../protos/shortened-url-response";
    import { browser } from "$app/environment";
  let url: string | null = null;
  let slug: string | undefined;
  let error_message: string | undefined;
  let prev_urls: Map<string, string> = new Map();
  $: error_message = undefined;
  $: slug = undefined;

  async function handleSubmit() {
    if (url == null) return;
    if ([...prev_urls.keys()].includes(url)) { slug = prev_urls.get(url); }
    error_message = undefined;
    let request = JSON.stringify({ url: url } as ShortenUrl);
    await fetch("/api/generate-url", {
      method: "POST",
      body: request,
      headers: { "Content-Type": "application/json" },
    })
      .then(async (_res) => ShortenedResponse.fromJson(await _res.json()))
      .then((res) => {
        slug = res.slug;
        error_message = res?.errorMessage;
        slug != null && prev_urls.set(url as string, slug);
      })
      .catch((e) => {
        console.error(e);
        error_message = "Something went wrong shortening the URL!";
      });
  }
</script>

<svelte:head>
  <link rel="preconnect" href="https://fonts.googleapis.com" />
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
  <link
    href="https://fonts.googleapis.com/css2?family=Inter:wght@100..900&display=swap"
    rel="preload"
    as="font"
  />
</svelte:head>

<form
  method="POST"
  action="/api/generate-url"
  on:submit|preventDefault={handleSubmit}
>
  <input
    type="url"
    name="url"
    aria-label="URL to Shorten"
    placeholder="https://example.com"
    bind:value={url}
    on:keyup={() => (url?.length == 0 ? (url = null) : {})}
  />
  <button type="submit" disabled={url == null}
    >{url != null ? "SHORTEN ME!" : "ENTER A LINK FIRST!"}</button
  >
  <span
    class={error_message != undefined || slug == null ? "error" : "success"}
  >
    {#if browser && error_message == undefined && slug != undefined}
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"
        ><!--!Font Awesome Free 6.5.2 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2024 Fonticons, Inc.--><path
          d="M256 512A256 256 0 1 0 256 0a256 256 0 1 0 0 512zM369 209L241 337c-9.4 9.4-24.6 9.4-33.9 0l-64-64c-9.4-9.4-9.4-24.6 0-33.9s24.6-9.4 33.9 0l47 47L335 175c9.4-9.4 24.6-9.4 33.9 0s9.4 24.6 0 33.9z"
        /></svg
      >
      Successfully created shortened link at<wbr /><a
        href="{location.origin}/{slug}">{location.origin}/{slug}</a
      >
    {:else if error_message != undefined }
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"
        ><!--!Font Awesome Free 6.5.2 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2024 Fonticons, Inc.--><path
          d="M256 32c14.2 0 27.3 7.5 34.5 19.8l216 368c7.3 12.4 7.3 27.7 .2 40.1S486.3 480 472 480H40c-14.3 0-27.6-7.7-34.7-20.1s-7-27.8 .2-40.1l216-368C228.7 39.5 241.8 32 256 32zm0 128c-13.3 0-24 10.7-24 24V296c0 13.3 10.7 24 24 24s24-10.7 24-24V184c0-13.3-10.7-24-24-24zm32 224a32 32 0 1 0 -64 0 32 32 0 1 0 64 0z"
        /></svg
      >
      {error_message}
    {/if}
  </span>
</form>

<style>
  :root {
    --__light-blue-background-r: 79;
    --__light-blue-background-g: 191;
    --__light-blue-background-b: 227;

    --__light-blue-foreground-r: 14;
    --__light-blue-foreground-g: 70;
    --__light-blue-foreground-b: 88;

    --light-blue-background: var(--__light-blue-background-r),
      var(--__light-blue-background-g), var(--__light-blue-background-b);

    --light-blue-foreground: var(--__light-blue-foreground-r),
      var(--__light-blue-foreground-g), var(--__light-blue-foreground-b);

    --url-input-background: #ffffff;
    --url-input-placeholder: #636363;

    --button-generic-hover: brightness(95%);
    --button-generic-active: brightness(90%);

    --success-foreground: #1e845f;
    --error-foreground: #cf7878;
  }

  * {
    font-family: "Inter", sans-serif;
  }

  form {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  form input {
    font-size: 1rem;

    width: 80%;
    max-width: 600px;
    padding: 1.5rem 1.25rem;

    border: 0;
    border-radius: 8px;
    outline: 1px solid var(--url-input-placeholder);

    background: var(--url-input-background);
  }

  form input::placeholder {
    color: var(--url-input-placeholder);
  }

  form input:active,
  form input:focus {
    outline-color: rgba(var(--light-blue-foreground), 0.4);
  }

  form button {
    font-weight: 900;
    font-size: 1rem;

    width: clamp(150px, 50%, 275px);
    height: 4rem;
    padding: 1rem;
    border-radius: 8px;

    background: rgba(var(--light-blue-background));
    color: rgba(var(--light-blue-foreground));

    outline: none;
    border: 0;

    cursor: pointer;
  }

  form button:disabled {
    cursor: auto;
    filter: contrast(60%) brightness(90%);
  }

  form button:hover:enabled {
    filter: var(--button-generic-hover);
  }

  form button:active:enabled {
    filter: var(--button-generic-active);
  }

  form .success,
  form .error {
    font-weight: 600;
    font-size: 1.1rem;

    display: flex;
    gap: 10px;
  }

  form .success {
    color: var(--success-foreground);
  }
  form .success a {
    color: var(--success-foreground);
    filter: brightness(110%);
  }

  form .error {
    color: var(--error-foreground);
  }

  form svg {
    width: 24px;
    height: 24px;
    vertical-align: middle;
  }

  form .success svg {
    fill: var(--success-foreground);
  }

  form .error svg {
    fill: var(--error-foreground);
  }
</style>
