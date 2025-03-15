import axios from 'axios';

//Setting the backend address
const backendAddr = 'http://127.0.0.1:8001';

export default async function lallpaperApi(path, data, crud) {
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
		return await operations[crud]().then((res) => res.data);
	} catch (error) {
		return error;
	}
}
