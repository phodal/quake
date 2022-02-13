import dayjs from "dayjs";

function format(str) {
  return dayjs(str * 1000).format('YYYY-MM-DD');
}

export default format;
