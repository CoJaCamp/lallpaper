<script>
	import { ArrowRight } from 'lucide-svelte';
</script>

<div class="flex h-screen w-[100rem] flex-col items-center">
	<div class="m-3 w-[40rem] border-2 border-[#c6b7be] bg-[#565a75] p-3 text-[#fafbf6] shadow-lg">
		<p class="mb-5 flex justify-center text-7xl">lallpaper</p>
		<hr class="text-[#c6b7be]" />
		<h1 class="my-1 flex justify-center text-4xl">A simple, minimalistic wallpaper repository</h1>
	</div>

	<div class="m-3 w-[40rem] border-2 border-[#c6b7be] bg-[#565a75] p-3 text-[#fafbf6] shadow-lg">
		<h2 class="mb-2 flex justify-center text-4xl">About</h2>
		<hr class="text-[#c6b7be]" />
		<p class="flex text-3xl">
			lallpaper is a simple wallpaper repository, to learn more about how everything works, click
			the header above.
		</p>
	</div>
	<div>
		<a href="/feed">
			<button
				class="m-3 flex w-[20rem] flex-row justify-center border-2 border-[#c6b7be] bg-[#565a75] p-3 text-5xl text-[#fafbf6] shadow-lg hover:bg-[#3f445d]"
			>
				Start

				<ArrowRight size="55" />
			</button>
		</a>
	</div>
</div>
