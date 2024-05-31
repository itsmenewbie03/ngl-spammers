<script lang="ts">
  import toast from "svelte-french-toast";

  let target: string = "";
  let message: string = "";
  let count: number = 0;
  let sent_count: number = 0;
  let random_messages: boolean = false;

  const clear = (event: Event) => {
    event.preventDefault();
    target = "";
    message = "";
    sent_count = 0;
  };

  const validate_url = (url: string) => {
    // INFO: we're parsing the url input
    // this returns the user name if the url is valid
    return /(?<=https:\/\/ngl\.link\/).*/.exec(url)?.[0];
  };

  const start = (event: Event) => {
    event.preventDefault();
    if (target === "") {
      toast.error("Please specify a target!");
      return;
    }
    if (!random_messages && message === "") {
      toast.error("Please fill all the fields!");
      return;
    }
    if (!count) {
      toast.error("Count must be at least 1!");
      return;
    }
    const username = validate_url(target);
    if (!username) {
      toast.error("Invalid ngl url!");
      return;
    }
    // INFO: we can now call rust here using tauri
    // for now we will just log what we have
    console.log({ username, message, count, random_messages });
  };

  const toggle = (event: Event) => {
    random_messages = (<HTMLInputElement>event.target).checked;
  };
</script>

<div class="flex w-full justify-center mt-8">
  <div class="card w-96 bg-base-100 shadow-xl bordered">
    <div class="card-body">
      <h2 class="text-4xl card-title">Let's get started!</h2>
      <label class="form-control w-full max-w-xs">
        <div class="label">
          <span class="label-text">Who to annoy?</span>
        </div>
        <input
          type="text"
          placeholder="Enter ngl url ..."
          class="input input-bordered w-full max-w-xs"
          bind:value={target}
        />
      </label>
      <label class="label cursor-pointer">
        <span class="label-text">Random messages?</span>
        <input
          type="checkbox"
          class="toggle toggle-primary"
          on:click={toggle}
        />
      </label>
      <!-- NOTE: We only show the message field if the user doesn't want random messages -->
      {#if !random_messages}
        <label class="form-control w-full max-w-xs">
          <div class="label">
            <span class="label-text">What to send?</span>
          </div>
          <input
            type="text"
            placeholder="Type your message here ..."
            class="input input-bordered w-full max-w-xs"
            bind:value={message}
          />
        </label>
      {/if}

      <label class="form-control w-full max-w-xs">
        <div class="label">
          <span class="label-text">How many?</span>
        </div>
        <input
          type="number"
          max="20"
          min="1"
          placeholder="Type your message here ..."
          class="input input-bordered w-full max-w-xs"
          bind:value={count}
        />
      </label>

      <label class="form-control w-full max-w-xs">
        <div class="label">
          <span class="label-text">Counter</span>
        </div>
        <input
          type="text"
          placeholder="Nothing is sent yet ..."
          class="input input-bordered input-disabled w-full max-w-xs"
          disabled
          bind:value={sent_count}
        />
      </label>
      <div class="card-actions justify-end">
        <button class="btn" on:click={clear}>Clear Form</button>
        <button class="btn btn-primary" on:click={start}>Start Spamming</button>
      </div>
    </div>
  </div>
</div>
