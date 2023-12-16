<script>
    import ListVocabs from '$lib/components/ListVocabs.svelte';
    import { invoke } from '@tauri-apps/api/tauri';
    import { Button } from "$lib/components/ui/button";
    import { Input } from "$lib/components/ui/input";
    import { Label } from "$lib/components/ui/label";
    import * as Card from "$lib/components/ui/card";

  
    let vocab_title = '';
    let vocab_content = '';
    let response = '';
  
    async function addVocab() {
      try {
        response = await invoke('post_vocab', { data: { title: vocab_title, content: vocab_content } });
      } catch (error) {
        console.error('Error invoking post_vocab:', error);
      }
    }
</script>

<Card.Root class="w-[350px]">
  <Card.Header>
    <Card.Title>Add Vocab</Card.Title>
    <Card.Description></Card.Description>
  </Card.Header>
  <Card.Content>
    <form>
      <div class="grid w-full items-center gap-4">
        <div class="flex flex-col space-y-1.5">
          <Label for="vocab-title">Title</Label>
          <Input name="vocab-title" id="vocab-title" placeholder="Enter a vocab title..." bind:value="{vocab_title}" />
        </div>
        <div class="flex flex-col space-y-1.5">
          <Label for="vocab-content">Content</Label>
          <Input name="vocab-content" id="vocab-content" placeholder="Enter a vocab content..." bind:value="{vocab_content}" />
        </div>
      </div>
    </form>
  </Card.Content>
  <Card.Footer class="flex justify-between">
    <Button variant="outline">Cancel</Button>
    <Button on:click="{() => addVocab}">Add</Button>
  </Card.Footer>
</Card.Root>

{#key response}
  <ListVocabs />
{/key}
