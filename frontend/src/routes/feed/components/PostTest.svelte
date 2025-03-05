<script>
	let postTest = [
		{
			id: 1,
			tags: ['green', 'purple', 'nier', 'dark', 'minimalism', 'yorha', 'nier:automata'],
			Uploader: 'user1',
			Favorites: 21,
			artist: 'Unknown',
			link: 'https://w.wallhaven.cc/full/kx/wallhaven-kxpyy1.png'
		},
		{
			id: 2,
			tags: ['big ben', 'clock tower', 'uk', 'landmark', 'london'],
			Uploader: 'user2',
			Favorites: 31,
			artist: 'Joe Man',
			link: 'https://w.wallhaven.cc/full/gj/wallhaven-gjd2z7.jpg'
		},
		{
			id: 3,
			tags: ['science', 'quote', 'text', 'typography', 'digital art', 'simple'],
			Uploader: 'user1',
			Favorites: 18,
			artist: 'JohnMan2',
			link: 'https://w.wallhaven.cc/full/ne/wallhaven-nexddr.jpg'
		},
		{
			id: 4,
			tags: ['green', 'purple', 'nier', 'dark', 'minimalism', 'yorha', 'nier:automata'],
			Uploader: 'user1',
			Favorites: 21,
			artist: 'Unknown',
			link: 'https://w.wallhaven.cc/full/kx/wallhaven-kxpyy1.png'
		},
		{
			id: 5,
			tags: ['big ben', 'clock tower', 'uk', 'landmark', 'london'],
			Uploader: 'user2',
			Favorites: 31,
			artist: 'Joe Man',
			link: 'https://w.wallhaven.cc/full/gj/wallhaven-gjd2z7.jpg'
		},
		{
			id: 22,
			tags: ['science', 'quote', 'text', 'typography', 'digital art', 'simple'],
			Uploader: 'user1',
			Favorites: 18,
			artist: 'JohnMan2',
			link: 'https://w.wallhaven.cc/full/ne/wallhaven-nexddr.jpg'
		},
		{
			id: 122,
			tags: ['green', 'purple', 'nier', 'dark', 'minimalism', 'yorha', 'nier:automata'],
			Uploader: 'user1',
			Favorites: 21,
			artist: 'Unknown',
			link: 'https://w.wallhaven.cc/full/kx/wallhaven-kxpyy1.png'
		},
		{
			id: 32,
			tags: ['big ben', 'clock tower', 'uk', 'landmark', 'london'],
			Uploader: 'user2',
			Favorites: 31,
			artist: 'Joe Man',
			link: 'https://w.wallhaven.cc/full/gj/wallhaven-gjd2z7.jpg'
		},
		{
			id: 63,
			tags: ['science', 'quote', 'text', 'typography', 'digital art', 'simple'],
			Uploader: 'user1',
			Favorites: 18,
			artist: 'JohnMan2',
			link: 'https://w.wallhaven.cc/full/ne/wallhaven-nexddr.jpg'
		},
		{
			id: 7,
			tags: ['green', 'purple', 'nier', 'dark', 'minimalism', 'yorha', 'nier:automata'],
			Uploader: 'user1',
			Favorites: 21,
			artist: 'Unknown',
			link: 'https://w.wallhaven.cc/full/kx/wallhaven-kxpyy1.png'
		},
		{
			id: 1234,
			tags: ['big ben', 'clock tower', 'uk', 'landmark', 'london'],
			Uploader: 'user2',
			Favorites: 31,
			artist: 'Joe Man',
			link: 'https://w.wallhaven.cc/full/gj/wallhaven-gjd2z7.jpg'
		},
		{
			id: 22223,
			tags: ['science', 'quote', 'text', 'typography', 'digital art', 'simple'],
			Uploader: 'user1',
			Favorites: 18,
			artist: 'JohnMan2',
			link: 'https://w.wallhaven.cc/full/ne/wallhaven-nexddr.jpg'
		},
		{
			id: 323,
			tags: ['green', 'purple', 'nier', 'dark', 'minimalism', 'yorha', 'nier:automata'],
			Uploader: 'user1',
			Favorites: 21,
			artist: 'Unknown',
			link: 'https://w.wallhaven.cc/full/kx/wallhaven-kxpyy1.png'
		},
		{
			id: 83,
			tags: ['big ben', 'clock tower', 'uk', 'landmark', 'london'],
			Uploader: 'user2',
			Favorites: 31,
			artist: 'Joe Man',
			link: 'https://w.wallhaven.cc/full/gj/wallhaven-gjd2z7.jpg'
		},
		{
			id: 12343,
			tags: ['science', 'quote', 'text', 'typography', 'digital art', 'simple'],
			Uploader: 'user1',
			Favorites: 18,
			artist: 'JohnMan2',
			link: 'https://w.wallhaven.cc/full/ne/wallhaven-nexddr.jpg'
		},
		{
			id: 1212,
			tags: ['green', 'purple', 'nier', 'dark', 'minimalism', 'yorha', 'nier:automata'],
			Uploader: 'user1',
			Favorites: 21,
			artist: 'Unknown',
			link: 'https://w.wallhaven.cc/full/kx/wallhaven-kxpyy1.png'
		},
		{
			id: 237,
			tags: ['big ben', 'clock tower', 'uk', 'landmark', 'london'],
			Uploader: 'user2',
			Favorites: 31,
			artist: 'Joe Man',
			link: 'https://w.wallhaven.cc/full/gj/wallhaven-gjd2z7.jpg'
		},
		{
			id: 982,
			tags: ['science', 'quote', 'text', 'typography', 'digital art', 'simple'],
			Uploader: 'user1',
			Favorites: 18,
			artist: 'JohnMan2',
			link: 'https://w.wallhaven.cc/full/ne/wallhaven-nexddr.jpg'
		}
	];
</script>

<div class="flex w-[80%] flex-wrap items-center justify-evenly">
	{#each postTest as post, index (post.id)}
		{#if (Math.floor(index / 3) % 3 === 0 && index % 3 === 0) || (Math.floor(index / 3) % 3 === 1 && index % 3 === 1) || (Math.floor(index / 3) % 3 === 2 && index % 3 === 2)}
			<div class="mb-[1%] flex h-[32%] w-[45%] border-4 border-[#c6b7be]">
				<img
					src={post.link}
					alt="This image contains: {post.id}"
					class="h-full w-full overflow-hidden object-cover"
				/>
			</div>
		{:else}
			<div class="mb-[1%] h-[32%] w-[26%] border-4 border-[#c6b7be]">
				<img
					src={post.link}
					alt="This image contains: {post.id}"
					class="h-full w-full overflow-hidden object-cover"
				/>
			</div>
		{/if}
	{/each}
</div>
