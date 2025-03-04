<script>
	import '../app.css';
	let { children } = $props();
	import { page } from '$app/state';
</script>

<div class=" flex justify-center">
	{#if page.url.pathname !== '/'}
		<nav class="fixed my-3 flex w-[75%] border-2 border-[#c6b7be] bg-[#565a75] to-75%">
			<h1 class="flex justify-start">
				<a href="/"><button class="mx-3 text-3xl hover:text-[#c6b7be]">lallpaper</button></a>
			</h1>
			<div class="flex w-full justify-end">
				<a href="/about"><button class="mx-3 text-3xl hover:text-[#c6b7be]">About</button></a>
			</div>
		</nav>
	{/if}
	<div class="mt-14">
		{@render children()}
	</div>
</div>
