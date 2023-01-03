export const guidGenerator = () => {
  const S4 = () => (((1+Math.random())*0x10000)|0).toString(16).substring(1)
  return (S4()+S4()+"-"+S4()+"-"+S4()+"-"+S4()+"-"+S4()+S4()+S4());
}

export const format_json = (data: string) =>
  JSON.stringify(JSON.parse(data), null, 2);