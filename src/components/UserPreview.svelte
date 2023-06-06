<script lang="ts">
  export let user: User | null;
  export let activity: Activity;
  export let activityName: string;
  activity;
</script>

{#if user}
  <div
    class="flex max-w-320px w-100% bg-mantle rounded-10px p-5px items-center font-sans flex-col"
  >
    <img
      src="https://cdn.discordapp.com/avatars/{user.id}/{user.avatar}.png?size=256"
      alt="{user.username}#{user.discriminator}'s Profile"
      class="w-50% rounded-full"
    />
    <div>
      <span class="text-2xl font-bold flex mx-2"
        >{user.username}#{user.discriminator}</span
      >
    </div>
    <div class="w-100%">
      {#if activityName}
        <p class="font-semibold my-2px">
          Playing <span>{activityName}</span>
        </p>
      {/if}
    </div>
  </div>
{:else}
  <h1 class="text-center">handshake not completed</h1>
{/if}
