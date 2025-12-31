function setupLivesheet() {
  defineWindows(60,80);
  formatPixels(60,80);
  
}

function defineWindows(needed_rows,needed_cols){

  const sheet = SpreadsheetApp.getActiveSheet();
  const currentRows = sheet.getMaxRows;
  const currentColumns = sheet.getMaxColumns;

  if (currentRows < needed_rows) {
    sheet.insertRowsAfter(currentRows, needed_rows - currentRows);
  } else if (currentRows > needed_rows) {
    sheet.deleteRows(needed_rows + 1, currentRows - needed_rows);
  }

  if (currentColumns < needed_cols) {
    sheet.insertColumnsAfter(currentColumns, needed_cols - currentColumns);
  } else if (currentColumns > needed_cols) {
    sheet.deleteColumns(needed_cols + 1, currentColumns - needed_cols);
  }
}

function formatPixels(needed_rows, needed_cols) {

  const sheet = SpreadsheetApp.getActiveSheet();
  const range = sheet.getRange(1, 1, needed_rows, needed_cols);

  // Set cells color to gray background
  range.setBackground("#808080");

  range.clearContent();

  // Make cells square
  sheet.setRowHeights(1, needed_rows, 12);     // 12 px
  sheet.setColumnWidths(1, needed_cols, 12);   // 12 px

  // Remove gridlines for clean view
  sheet.setHiddenGridlines(true);
}
