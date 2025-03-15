<script>
	import lallpaperApi from '$lib/api';

	let loginData = $state({
		username: '',
		password: ''
	});

	let registerPasswordComf = $state('');
	let registerData = $state({
		username: '',
		password: ''
	});

	let login = $state(true);
	function handleRegisterLoginSwitch(e) {
		e.preventDefault();
		login == true ? (login = false) : (login = true);
	}

	function handleLogin() {
		lallpaperApi('/users/login', loginData, 'post');
	}

	async function handleRegister() {
		let response = await lallpaperApi('/users/register', registerData, 'post');
		console.log(response); // This will log the response after it's resolved
	}
</script>

<div
	class="fixed mb-[1%] h-[45%] min-h-[37rem] w-[36%] border-4 border-[#c6b7be] bg-[#565a75] text-4xl"
>
	{#if login}
		<form class="mt-[3%] mb-9 flex flex-col items-center">
			<h3 class="mt-3">Enter your username and password to login!</h3>
			<div class="mt-[15%]">
				<input
					class="text-center"
					type="text"
					placeholder="Username"
					id="usernameInput"
					bind:value={loginData.username}
				/>
			</div>
			<div>
				<input
					class="text-center"
					type="password"
					id="passwordInput"
					placeholder="Password"
					bind:value={loginData.password}
				/>
			</div>
			<button
				onclick={handleLogin}
				class="m-3 mt-15 mb-[10%] flex w-[10rem] flex-row justify-center border-2 border-[#c6b7be] bg-[#565a75] p-3 text-5xl text-[#fafbf6] shadow-lg hover:bg-[#3f445d] hover:text-[#c6b7be]"
			>
				Login
			</button>
			<p class=" text-2xl">
				Don't have an account?
				<button onclick={(e) => handleRegisterLoginSwitch(e)} class="underline">Register </button>
			</p>
		</form>
	{:else}
		<form class="mt-[3%] mb-9 flex flex-col items-center contain-content">
			<h3 class="mt-3">Enter your username and password to register!</h3>
			<div class="mt-[15%]">
				<input
					class="text-center"
					type="text"
					placeholder="Username"
					id="usernameInput"
					bind:value={registerData.username}
				/>
			</div>
			<div>
				<input
					class="text-center"
					type="password"
					id="passwordInput"
					placeholder="Password"
					bind:value={registerData.password}
				/>
			</div>
			<div>
				<input
					class="text-center"
					type="password"
					id="passwordInput"
					placeholder="Confirm password"
					bind:value={registerPasswordComf}
				/>
			</div>
			<button
				onclick={handleRegister}
				class="m-3 mt-20 flex w-[12rem] flex-row justify-center border-2 border-[#c6b7be] bg-[#565a75] p-3 text-5xl text-[#fafbf6] shadow-lg hover:bg-[#3f445d] hover:text-[#c6b7be]"
			>
				Register
			</button>
			<p class="text-2xl">
				Already have an account?
				<button onclick={handleRegisterLoginSwitch} class="underline">Login </button>
			</p>
		</form>
	{/if}
</div>
