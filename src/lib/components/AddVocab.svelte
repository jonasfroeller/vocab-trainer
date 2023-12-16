<script>
    import ListVocabs from '$lib/components/ListVocabs.svelte';
    import { invoke } from '@tauri-apps/api/tauri'
  
    let vocab = ''
    let response = ''
  
    async function addVocab() {
      try {
        response = await invoke('post_vocab', { data: { title: vocab, content: vocab } });
      } catch (error) {
        console.error('Error invoking post_vocab:', error);
      }
    }
</script>

<div>
    <input id="input" placeholder="Enter a vocab..." bind:value="{vocab}" />
    <button on:click="{() => addVocab}">add</button>
    <p>{response}</p>
</div>

{#key response}
  <ListVocabs />
{/key}
