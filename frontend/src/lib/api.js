import axios from 'axios';

const backendAddr = '127.0.0.1:8001';

export async function lallpaperApi(path, data, crud) {
	let fullPath = backendAddr + path;

	const operations = {
		get: () => axios.get(fullPath),
		post: () => axios.post(fullPath, data),
		delete: () => axios.delete(fullPath)
	};

	try {
		if (!operations[crud]) {
			throw new Error(`Unsupported operation: ${crud}`);
		}
		return await operations[crud]().then((res) => res.response);
	} catch (e) {
		return e;
	}
}
