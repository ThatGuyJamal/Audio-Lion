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

export function extractFileName(path: string): string {
	var startIndex =
		path.lastIndexOf("\\") >= 0 ? path.lastIndexOf("\\") : path.lastIndexOf("/");
	var filename = path.substring(startIndex);
	if (filename.indexOf("\\") === 0 || filename.indexOf("/") === 0) {
		filename = filename.substring(1);
	}
	var dotIndex = filename.lastIndexOf(".");
	if (dotIndex > -1) {
		filename = filename.substring(0, dotIndex);
	}
	return filename;
}

export function getFileExtension(filePath: string) {
	// Get the last dot position in the file path
	const lastDotIndex = filePath.lastIndexOf(".");

	// If there's no dot, or it's the first or last character, return an empty string
	if (
		lastDotIndex === -1 ||
		lastDotIndex === 0 ||
		lastDotIndex === filePath.length - 1
	) {
		return "";
	}

	// Get the file extension by taking the substring after the last dot
	const extension = filePath.substring(lastDotIndex + 1);

	return extension.toLowerCase(); // Return the extension in lowercase
}

export function getIndexByName(name: string, arr: string[]): number {
	return arr.findIndex((element) => element === name);
}

export function getDirectoryPath(
	filePath: string,
	platform: ValidPlatforms
): string {
	// Determine the path separator based on the platform
	let pathSeparator = "/";
	if (platform === "windows") {
		pathSeparator = "\\";
	} else if (platform === "mac" || platform === "linux") {
		pathSeparator = "/";
	}

	// Get the last index of the path separator
	const lastIndex = filePath.lastIndexOf(pathSeparator);

	// Extract the directory path
	const dirPath = filePath.slice(0, lastIndex + 1);

	return dirPath;
}

export type ValidPlatforms = "windows" | "mac" | "linux" | "unknown";

export function isValidDirectory2(
	path: string,
	platform: ValidPlatforms
): boolean {
	// Check the path separator based on the platform
	const separator = platform === "windows" ? "\\" : "/";
	const parts = path.split(separator);

	// Windows directory paths must start with a drive letter or UNC path
	if (platform === "windows" && parts.length > 0) {
		const firstPart = parts[0].toLowerCase();
		if (!/^[a-z]:$/.test(firstPart) && !/^\\\\[^\\]+\\[^\\]+/.test(path)) {
			return false;
		}
	}

	// Check that each part of the path is a valid directory name
	for (const part of parts) {
		// Windows directory names can't contain certain characters
		if (platform === "windows" && /[<>:"/\\|?*]/.test(part)) {
			return false;
		}
		// Directory names can't be empty or contain certain characters on any platform
		if (/^[.]{1,2}$/.test(part) || /[<>:"/\\|?*\0]/.test(part)) {
			return false;
		}
	}

	// The path is valid if we made it this far
	return true;
}

/**
 * Checks if a path is a valid directory path for the given platform.
 * @param path The path to check
 * @param platform The platform to check the path for
 * @returns True if the path is valid, false otherwise
 */
export function isValidDirectory(
	path: string,
	platform: ValidPlatforms
): boolean {
	// Check the path separator based on the platform
	const separator = platform === "windows" ? "\\" : "/";
	const parts = path.split(separator);

	// Check that the last part of the path is not a file name
	const lastPart = parts[parts.length - 1];
	if (/\.[^\.]+$/.test(lastPart)) {
		return false;
	}

	// Windows directory paths must start with a drive letter or UNC path
	if (platform === "windows" && parts.length > 0) {
		const firstPart = parts[0].toLowerCase();
		if (!/^[a-z]:$/.test(firstPart) && !/^\\\\[^\\]+\\[^\\]+/.test(path)) {
			return false;
		}
	}

	// Check that each part of the path is a valid directory name
	for (const part of parts) {
		// Windows directory names can't contain certain characters
		if (platform === "windows" && /[<>:"/\\|?*]/.test(part)) {
			return false;
		}
		// Directory names can't be empty or contain certain characters on any platform
		if (/^[.]{1,2}$/.test(part) || /[<>:"/\\|?*\0]/.test(part)) {
			return false;
		}
	}

	// The path is valid if we made it this far
	return true;
}

export function getCurrentPlatform(): ValidPlatforms {
	const userAgent = window.navigator.userAgent;
	if (userAgent.indexOf("Windows") !== -1) {
		return "windows";
	} else if (userAgent.indexOf("Linux") !== -1) {
		return "linux";
	} else if (userAgent.indexOf("Mac") !== -1) {
		return "mac";
	} else {
		return "unknown";
	}
}
