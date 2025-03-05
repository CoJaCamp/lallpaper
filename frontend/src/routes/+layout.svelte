<script>
	import '../app.css';
	let { children } = $props();
	import { page } from '$app/state';
</script>

<div class=" flex justify-center">
	{#if page.url.pathname !== '/'}
		<nav class="fixed my-3 flex h-12 w-[79%] border-2 border-[#c6b7be] bg-[#565a75] to-75%">
			<a href="/">
				<button class=" mx-3 flex justify-start text-3xl hover:text-[#c6b7be]">lallpaper</button>
			</a>
			<div class="flex w-full justify-end">
				<a href="/about"><button class="mx-3 text-3xl hover:text-[#c6b7be]">About</button></a>
			</div>
		</nav>
	{/if}
	<div class="mt-18">
		{@render children()}
	</div>
</div>
