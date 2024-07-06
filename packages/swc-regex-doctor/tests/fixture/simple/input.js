// write some example for regex search and match

const regex = /foo/g;
const str = 'foo foo';

regex.test(str)

str.match(regex)

str.replace(regex, 'bar')

let a = {
  test(){
    console.log('test')
  }
};
a.test();
