<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"

  let dir = ''
  let scanned: string[] = []

  async function scan(){
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    const ret: string = await invoke('scan', { dir })
    scanned = ret.split('\n')
  }
</script>

<div>
  <form class="row" on:submit|preventDefault={scan}>
    <input id="scan-input" placeholder="Enter dir path..." bind:value={dir} />
    <button type="submit">Scan</button>
  </form>
  <ol>
    {#each scanned as file, _index}
      <li>{file}</li>
    {/each}
  </ol>
</div>