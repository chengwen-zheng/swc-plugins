// write some example for regex search and match
const regex = /foo/g;
const str = 'foo foo';
{
  console.time("regex");
  regex.test(str);
  console.timeEnd("regex");
}
{
  console.time("regex");
  str.match(regex);
  console.timeEnd("regex");
}
{
  console.time("regex");
  str.replace(regex, 'bar');
  console.timeEnd("regex");
}
let a = {
  test(){
    console.log('test')
  }
};
a.test();
