<script>
    import { invoke } from '@tauri-apps/api/tauri';
    import { onMount } from 'svelte';
  
    let response = '';
  
    async function getAllVocabs() {
      try {
        response = await invoke('get_all_vocabs');
      } catch (error) {
        console.error('Error invoking get_all_vocabs:', error);
      }
    }
  
    onMount(async () => {
      await getAllVocabs();
    });
  </script>
  
  {response}
  