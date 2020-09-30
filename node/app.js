const { convert } = require('../pkg/ssvm_nodejs_starter_lib.js');
const { argv } = require('yargs')

if (argv.to == "arabic") {
  if (typeof (argv.value) == "string") {
    console.log(convert(String(argv.value), argv.to))
  } else {
    console.log(argv.value + " is a " + typeof (argv.value))
    console.log("For roman numeral->arabic conversion, use --value=[String] eg. XX along with --to=arabic")
  }
}
else if (argv.to == "roman") {
  if (Number.isInteger(argv.value)) {
    console.log(convert(String(argv.value), argv.to))
  } else {
    console.log(argv.value + " is a " + typeof (argv.value))
    console.log("For arabic numeral->roman conversion, use --value=[Integer] eg. 20 along with --to=roman")
  }
}
else {
  console.log(" RUN WITH:\n --value=[the value in roman or arabic format]\n --to=[arabic||roman]")
}
