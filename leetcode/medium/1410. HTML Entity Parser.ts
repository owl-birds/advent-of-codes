function entityParser(text: string): string {
  const special_charas: Map<string, string> = new Map();
  special_charas.set("&quot;", '"');
  special_charas.set("&apos;", "'");
  special_charas.set("&amp;", "&");
  special_charas.set("&gt;", ">");
  special_charas.set("&lt;", "<");
  special_charas.set("&frasl;", "/");
  const special_regex_g: RegExp = /&\w+;/g;
  const specials: string[] | null = text.match(special_regex_g);
  let result_text: string = text;
  if (!specials) return result_text;
  for (let i = 0; i < specials.length; i += 1) {
    if (!special_charas.has(specials[i])) continue;
    const temp_regex: RegExp = new RegExp(specials[i]);
    result_text = result_text.replace(
      temp_regex,
      special_charas.get(specials[i])!
    );
  }
  return result_text;
}
const entityParser_2 = (text: string): string => {
  return text
    .replace(/&quot;/g, '"')
    .replace(/&apos;/g, "'")
    .replace(/&gt;/g, ">")
    .replace(/&lt;/g, "<")
    .replace(/&frasl;/g, "/")
    .replace(/&amp;/g, "&");
};
