<script>
	import '../app.css';
	import { page } from '$app/state';
	import AccountComponent from './components/Account.svelte';
	let { children } = $props();

	let accountCompontentShow = $state(false);

	function handleBackdropClick(e) {
		if (e.target === e.currentTarget) {
			accountCompontentShow = false;
		}
	}
</script>

<div class=" flex justify-center">
	{#if page.url.pathname !== '/'}
		<nav class="fixed my-3 flex h-12 w-[79%] border-2 border-[#c6b7be] bg-[#565a75] to-75%">
			<a href="/">
				<button class=" mx-3 flex justify-start text-3xl hover:text-[#c6b7be]">lallpaper</button>
			</a>
			<div class="flex w-full justify-end">
				<a href="/about"><button class="mx-3 text-3xl hover:text-[#c6b7be]">About</button></a>

				<button
					class="mx-3 mb-2 text-3xl hover:text-[#c6b7be]"
					onclick={() => {
						accountCompontentShow = true;
						console.log('clicked');
					}}>Login/Register</button
				>
			</div>
		</nav>
	{/if}
	{#if accountCompontentShow}
		<button
			class="fixed flex h-screen w-screen items-center justify-center bg-[#0f0f1b]/75 backdrop-blur-md"
			onclick={handleBackdropClick}
			aria-label="Close login component backdrop"
		></button>

		<AccountComponent />
	{/if}
	<div class="mt-18">
		{@render children()}
	</div>
</div>
