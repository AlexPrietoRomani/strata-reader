import glob
import opendataloader_pdf

pdf_files = glob.glob("tests/fixtures/pdfs/articles/*.pdf")
print(f"Converting {len(pdf_files)} PDF files...")

opendataloader_pdf.convert(
    input_path=pdf_files,
    output_dir="tests/fixtures/salidas/opendataloader-pdf",
    format="markdown,json"
)
print("Done!")
