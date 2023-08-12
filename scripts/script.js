let theme = window.localStorage.currentTheme;

$("body").addClass(theme);


if ($("body").hasClass("light")) {
  $(".toggle").addClass("fa-moon-o");
  $(".toggle").removeClass("fa-sun-o");
} else {
  $(".toggle").removeClass("fa-moon-o");
  $(".toggle").addClass("fa-sun-o");
}

$(".toggle").click(function () {
  $(".toggle").toggleClass("fa-moon-o");
  $(".toggle").toggleClass("fa-sun-o");

  if ($("body").hasClass("light")) {
    $("body").toggleClass("light");
    localStorage.removeItem("currentTheme");
    localStorage.currentTheme = "dark";
  } else {
    $("body").toggleClass("light");
    localStorage.removeItem("currentTheme");
    localStorage.currentTheme = "light";
  }
});
