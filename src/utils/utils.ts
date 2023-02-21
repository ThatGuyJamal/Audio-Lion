export function CreateLink(url: string, name?:string): string {
    if (!name) name = "Click here"
    return `<a href="${url}" target="_blank" rel="noreferrer">${name}</a>`;
}