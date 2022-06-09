use pdfium_render::prelude::*;

fn main() -> Result<(), PdfiumError> {
    // For general comments about pdfium-render and binding to Pdfium, see export.rs.

    let pdfium = Pdfium::new(
        Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
            .or_else(|_| Pdfium::bind_to_system_library())?,
    );

    // There are several functions available to copy one or more pages from one document
    // to another:

    // PdfDocument::append(): this is the simplest. It copies all pages in one document
    // into this PdfDocument, placing the copied pages at the end of this PdfDocument's
    // PdfPages collection.

    // PdfPages::import_page_from_document(): copies one page from a document
    // into this PdfPages collection at a user-defined position.

    // PdfPages::import_page_range_from_document(): copies multiple pages, expressed
    // as a sequential 0-indexed inclusive range, from a document into this PdfPages
    // collection at a user-defined position.

    // PdfPages::import_pages_from_document(): copies multiple pages, expressed as
    // a "human-friendly" 1-indexed comma-delimited string of page numbers and ranges,
    // from a document into this PdfPages collection at a user-defined position.
    // The page range string is the same as what you'd expect to use in, e.g. a
    // Print File dialog box, with a specification like "1,3-4,6,9-12" being accepted.

    // All these functions are demonstrated below.

    // Create a new blank document...

    let document = pdfium.create_new_pdf()?;

    // ... append all pages from a test file using PdfDocument::append() ...

    document
        .pages()
        .append(&pdfium.load_pdf_from_file("test/text-test.pdf", None)?)?;

    // ... import some more pages from another test file, this time
    // using PdfPages::import_pages_from_document() ...

    document.pages().copy_pages_from_document(
        &pdfium.load_pdf_from_file("test/export-test.pdf", None)?,
        "3-6", // Note: 1-indexed, not 0-indexed
        document.pages().len(),
    )?;

    // ... import some more pages from yet another test file, this time
    // using PdfPages::import_page_range_from_document() ...

    document.pages().copy_page_range_from_document(
        &pdfium.load_pdf_from_file("test/form-test.pdf", None)?,
        0..=2, // Note: 0-indexed, inclusive range
        document.pages().len(),
    )?;

    // ... insert front and back cover pages, this time using PdfPages::import_page_from_document() ...

    document.pages().copy_page_from_document(
        &pdfium.load_pdf_from_file("test/export-test.pdf", None)?,
        0, // First page, i.e. front cover; note: 0-indexed
        0,
    )?;

    document.pages().copy_page_from_document(
        &pdfium.load_pdf_from_file("test/export-test.pdf", None)?,
        6, // Last page, i.e. back cover; note: 0-indexed
        document.pages().len(),
    )?;

    // ... remove the sixth page ...

    document.pages().delete_page_at_index(
        5, // 0-indexed
    )?;

    // ... and save the final result.

    document.save_to_file("test/concat-test.pdf")?;

    Ok(())
}
