<script>
	import '../app.css';
	let { children } = $props();
</script>

<div class=" flex justify-center">
	{@render children()}
</div>
