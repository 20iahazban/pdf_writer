//! This example shows you how to embed raster images into a PDF.

use pdf_writer::{Finish, Pdf, Rect, Ref};

fn main() -> std::io::Result<()> {
    // Start writing.
    let mut pdf = Pdf::new();

    // Define some indirect reference ids we'll use.
    let catalog_id = Ref::new(1);
    let page_tree_id = Ref::new(2);
    let page_id = Ref::new(3);

    let content_id = Ref::new(6);
    let metadata_id = Ref::new(7);

    // Set up the page tree. For more details see `hello.rs`.
    pdf.catalog(catalog_id).pages(page_tree_id);
    pdf.pages(page_tree_id).kids([page_id]).count(1);

    let metadata = r#"<?xpacket begin="﻿" id="W5M0MpCehiHzreSzNTczkc9d"?>
    <x:xmpmeta xmlns:x="adobe:ns:meta/" x:xmptk="Adobe XMP Core 5.6-c014 79.164499, 2016/09/14-01:09:01        ">
      <rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
        <rdf:Description rdf:about=""
            xmlns:dc="http://purl.org/dc/elements/1.1/"
            xmlns:xmp="http://ns.adobe.com/xap/1.0/"
            xmlns:custom="http://ns.example.com/custom/1.0/">
          <dc:creator>
            <rdf:Seq>
              <rdf:li>John Doe</rdf:li>
            </rdf:Seq>
          </dc:creator>
          <dc:title>
            <rdf:Alt>
              <rdf:li xml:lang="x-default">Sample Title</rdf:li>
            </rdf:Alt>
          </dc:title>
          <dc:description>
            <rdf:Alt>
              <rdf:li xml:lang="x-default">credit card visa: 4041598475929.</rdf:li>
            </rdf:Alt>
          </dc:description>
          <xmp:CreateDate>2024-07-02T12:00:00Z</xmp:CreateDate>
          <custom:CustomField>Custom Value</custom:CustomField>
        </rdf:Description>
      </rdf:RDF>
    </x:xmpmeta>
    <?xpacket end="r"?>"#;

    let mut page = pdf.page(page_id);
    page.metadata(metadata_id);
    let a4 = Rect::new(0.0, 0.0, 595.0, 842.0);
    page.media_box(a4);
    page.parent(page_tree_id);
    page.contents(content_id);

    page.finish();

    pdf.metadata(metadata_id, metadata.as_bytes());

    std::fs::write("target/blank_page_withmetadata.pdf", pdf.finish())
}
