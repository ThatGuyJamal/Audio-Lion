import { AudioFileType } from "$lib/types/AppConfig";

export function formatFileSize(sizeInBytes: number): string {
	const kb = 1024;
	const mb = kb * 1024;
	const gb = mb * 1024;
	const tb = gb * 1024;

	if (sizeInBytes < kb) {
		return sizeInBytes + " B";
	} else if (sizeInBytes < mb) {
		return (sizeInBytes / kb).toFixed(2) + " KB";
	} else if (sizeInBytes < gb) {
		return (sizeInBytes / mb).toFixed(2) + " MB";
	} else if (sizeInBytes < tb) {
		return (sizeInBytes / gb).toFixed(2) + " GB";
	} else {
		return (sizeInBytes / tb).toFixed(2) + " TB";
	}
}

export function capitalizeFirstLetter(str: string): string {
	if (str.length === 0) {
		return str;
	}
	return str.charAt(0).toUpperCase() + str.slice(1);
}

export function extractFileNameWithoutExtension({
	fileName,
	fileType,
	capitalize,
}: {
	fileName: string;
	fileType?: AudioFileType;
	capitalize?: boolean;
}): string {
	const validFileTypes = Object.values(AudioFileType);

	if (fileType && !validFileTypes.includes(fileType)) {
		throw new Error(`Invalid file type: ${fileType}`);
	}

	const extensions = fileType
		? [fileType]
		: [AudioFileType.MP3, AudioFileType.WAV];
	const extensionLength = fileType ? fileType.length + 1 : 4;

	const nameWithoutExtension = fileName.slice(0, -extensionLength);

	return capitalize
		? capitalizeFirstLetter(nameWithoutExtension)
		: nameWithoutExtension;
}