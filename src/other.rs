#![feature(box_patterns)]

use swc_common::{self, sync::Lrc, FileName, SourceMap};
use swc_ecma_ast as ast;
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax, TsConfig};

trait CollectFromJsxNode {
    fn collect(&self, node: &ast::JSXElement, collection: &mut Vec<Self>) -> Vec<Self>
    where
        Self: Sized;
}

struct Header {
    msg: String,
}

fn traverse<T>(expr: &mut ast::Expr) -> T {
    match expr {
        ast::This(expr) => This(e.clone()),
        ast::Array(expr) => Array(e.clone()),
        ast::Object(expr) => Object(e.clone()),
        ast::Fn(expr) => Fn(e.clone()),
        ast::Unary(expr) => Unary(e.clone()),
        ast::Update(expr) => Update(e.clone()),
        ast::Bin(expr) => Bin(e.clone()),
        ast::Assign(expr) => Assign(e.clone()),
        ast::Member(expr) => Member(e.clone()),
        ast::SuperProp(expr) => SuperProp(e.clone()),
        ast::Cond(expr) => Cond(e.clone()),
        ast::Call(expr) => Call(e.clone()),
        ast::New(expr) => New(e.clone()),
        ast::Seq(expr) => Seq(e.clone()),
        ast::Ident(expr) => Ident(e.clone()),
        ast::Lit(expr) => Lit(e.clone()),
        ast::Tpl(expr) => Tpl(e.clone()),
        ast::TaggedTpl(expr) => TaggedTpl(e.clone()),
        ast::Arrow(expr) => Arrow(e.clone()),
        ast::Class(expr) => Class(e.clone()),
        ast::Yield(expr) => Yield(e.clone()),
        ast::MetaProp(expr) => MetaProp(e.clone()),
        ast::Await(expr) => Await(e.clone()),
        ast::Paren(expr) => Paren(e.clone()),
        ast::JSXMember(expr) => JSXMember(e.clone()),
        ast::JSXNamespacedName(expr) => JSXNamespacedName(e.clone()),
        ast::JSXEmpty(expr) => JSXEmpty(e.clone()),
        ast::JSXElement(expr) => JSXElement(e.clone()),
        ast::JSXFragment(expr) => JSXFragment(e.clone()),
        ast::TsTypeAssertion(expr) => TsTypeAssertion(e.clone()),
        ast::TsConstAssertion(expr) => TsConstAssertion(e.clone()),
        ast::TsNonNull(expr) => TsNonNull(e.clone()),
        ast::TsAs(expr) => TsAs(e.clone()),
        ast::TsInstantiation(expr) => TsInstantiation(e.clone()),
        ast::PrivateName(expr) => PrivateName(e.clone()),
        ast::OptChain(expr) => OptChain(e.clone()),
        ast::Invalid(expr) => Invalid(e.clone()),
    }
}

fn traverse_jsx_tree<'a, T>(branches: &'a Vec<ast::JSXElementChild>, v: &mut Vec<T>) -> () {
    for branch in branches {
        match branch {
            &ast::JSXElementChild::JSXElement(box ref jsx) => {
                traverse_jsx_tree(&jsx.children, v);
            }
            _ => {}
        }
    }
}

fn collect_from_jsx_nodes<Collection>(stmt: ast::Stmt) -> ()
where
    Collection: Sized, /*+ CollectFromJsxNode*/
{
    let mut v = Vec::<Collection>::new();

    match stmt {
        ast::Stmt::Expr(expr_stmt) => match expr_stmt.expr {
            box ast::Expr::JSXElement(box ast::JSXElement {
                opening,
                children,
                closing,
                ..
            }) => {
                // print!("{:?}", opening);
                // print!("{:?}", closing);
                traverse_jsx_tree(&children, &mut v);
            }
            box ast::Expr::JSXEmpty(jsx_empty) => (println!("JSXEmpty")),
            box ast::Expr::JSXFragment(jsx_frag) => (println!("JSXFrag")),
            box ast::Expr::JSXMember(jsx_member) => (println!("JSXMember")),
            none @ _ => (println!("None {:?}", none)),
        },
        not_statement @ _ => (println!("not statemeent")),
    }
    // return v;
}

fn main() {
    let cm: Lrc<SourceMap> = Default::default();

    let src = r#"
    import * as Card from '@/components/display/card';
    import { Definition } from '@/components/display/definition';
    import { ObjectiveTable } from '@/components/display/tables/objective-table';
    import * as Tabs from '@/components/navigation/tabs';
    import { useReferences } from '@/hooks/useReferences';
    import { HomeLayout } from '@/layouts/home';
    import { Fragment, ReactNode } from 'react';
    import { Flex } from '@/components/layout/flex';
    import { usePublicAssets } from '@/hooks/usePublicAssets';
    import * as Grid from '@/components/layout/grid';
    
    const Page = () => {
      const FOOTNOTES_ID = 'footnote_block';
    
      const [Ref, Footnotes] = useReferences(FOOTNOTES_ID, [
        'Zachary C, et al. Lasers Surg Med. 2021;53: 70-78',
      ]);
    
      const [
        day11A,
        day11B,
        day11C,
        day11D,
        day17A,
        day17B,
        day17C,
        day17D,
        stainingA,
        stainingB,
        stainingC,
        stainingD,
      ] = usePublicAssets([
        'ALL120_Platform-2_Section-1_Sub-2_2d_IMAGE_11-Days_A.jpg',
        'ALL120_Platform-2_Section-1_Sub-2_2d_IMAGE_11-Days_B.jpg',
        'ALL120_Platform-2_Section-1_Sub-2_2d_IMAGE_11-Days_C.jpg',
        'ALL120_Platform-2_Section-1_Sub-2_2d_IMAGE_11-Days_D.jpg',
        'ALL120_Platform-2_Section-1_Sub-2_2d_IMAGE_17-Days_A.jpg',
        'ALL120_Platform-2_Section-1_Sub-2_2d_IMAGE_17-Days_B.jpg',
        'ALL120_Platform-2_Section-1_Sub-2_2d_IMAGE_17-Days_C.jpg',
        'ALL120_Platform-2_Section-1_Sub-2_2d_IMAGE_17-Days_D.jpg',
        'ALL120_Platform-2_Section-1_Sub-2_2d_IMAGE_17-Days_Tissue-Staining_A.jpg',
        'ALL120_Platform-2_Section-1_Sub-2_2d_IMAGE_17-Days_Tissue-Staining_B.jpg',
        'ALL120_Platform-2_Section-1_Sub-2_2d_IMAGE_17-Days_Tissue-Staining_C.jpg',
        'ALL120_Platform-2_Section-1_Sub-2_2d_IMAGE_17-Days_Tissue-Staining_D.jpg',
      ]);
    
      return (
        <Fragment>
          <h1 id={`Adipose Tissue Response`}>
            Adipose Tissue Response (Zachary, et al. 2020)
          </h1>
          <ObjectiveTable
            objective={
              <p>
                Assess adipose tissue response to electromagnetic muscle stimulation
                (EMMS) vs cryolipolysis
              </p>
            }
            studyDesign={
              <ul>
                <li>
                  Patient undergoing abdominoplasty underwent treatment with a
                  commercially available (n = 6) EMMS, prototype EMMS (n = 3), or
                  cryolipolysis (n = 2)
                  <ul>
                    <li>
                      EMMS single treatment with Emsculpt or CoolTone Prototype
                      (100% intensity for 30 min)
                    </li>
                    <li>
                      Cryolipolysis single treatment with CoolSculpting using
                      CoolAdvantage (-11°C, 35 min)
                    </li>
                  </ul>
                </li>
                <li>
                  Superficial and deep subcutaneous adipose tissue was harvested
                  post-treatment
                </li>
    
                <li>
                  {`Hematoxylin and eosin (H&E) staining was performed to look for
                  inflammatory response`}
                </li>
    
                <li>
                  Irreversible fat injury was assessed with perilipin
                  immunofluorescence
                </li>
              </ul>
            }
            results={
              <ul>
                <li>
                  {`No EMMS-treated samples showed an inflammatory response on H&E
                  analysis at 3, 10, 11, or 17 days post-treatment`}
                  <ul>
                    <li>
                      Perilipin persisted in EMMS-treated samples, indicating that
                      all fat cells were viable
                    </li>
                  </ul>
                </li>
                <li>
                  {`Cryolipolysis-treated samples revealed a marked inflammatory
                  response on H&E analysis`}
                  <ul>
                    <li>
                      Loss of perilipin staining was clear evidence of irreversible
                      fat cell injury in cryolipolysis-treated adipose tissue
                    </li>
                  </ul>
                </li>
              </ul>
            }
            safety={
              <p>
                Safety was monitored by documentation of AEs throughout the active
                treatment portion of the study
              </p>
            }
          />
          <Definition
            word="EMMS"
            definition={<span>electromagnetic muscle stimulation.</span>}
          />
    
          <h2 id={`Fat Cell Injury With Cryolipolysis vs EMMS`}>
            Fat Cell Injury With Cryolipolysis vs EMMS
          </h2>
    
          <Grid.Container columns={4}>
            <Card.Root>
              <Card.Media type={'image'} src={day11A} />
            </Card.Root>
            <Card.Root>
              <Card.Media type={'image'} src={day11B} />
            </Card.Root>
            <Card.Root>
              <Card.Media type={'image'} src={day11C} />
            </Card.Root>
            <Card.Root>
              <Card.Media type={'image'} src={day11D} />
            </Card.Root>
          </Grid.Container>
          <p>
            <cite> Harvested 11 days post treatment</cite>
          </p>
    
          <Tabs.Root defaultValue="tab1" orientation="vertical">
            <Tabs.List aria-label="tabs example">
              <Tabs.Trigger value="tab1">
                <em> {'H&E Tissue Straining'}</em>
              </Tabs.Trigger>
              <Tabs.Trigger value="tab2">
                <em> Tissue Straining</em>
              </Tabs.Trigger>
            </Tabs.List>
            <Tabs.Content value="tab1">
              <Grid.Container columns={4}>
                <Card.Root>
                  <Card.Media type={'image'} src={day17A} />
                </Card.Root>
                <Card.Root>
                  <Card.Media type={'image'} src={day17B} />
                </Card.Root>
                <Card.Root>
                  <Card.Media type={'image'} src={day17C} />
                </Card.Root>
                <Card.Root>
                  <Card.Media type={'image'} src={day17D} />
                </Card.Root>
              </Grid.Container>
            </Tabs.Content>
            <Tabs.Content value="tab2">
              <Grid.Container columns={4}>
                <Card.Root>
                  <Card.Media type={'image'} src={stainingA} />
                </Card.Root>
                <Card.Root>
                  <Card.Media type={'image'} src={stainingB} />
                </Card.Root>
                <Card.Root>
                  <Card.Media type={'image'} src={stainingC} />
                </Card.Root>
                <Card.Root>
                  <Card.Media type={'image'} src={stainingD} />
                </Card.Root>
              </Grid.Container>
            </Tabs.Content>
          </Tabs.Root>
          <p>
            <cite> Harvested 17 days post treatment</cite>
          </p>
    
          <Footnotes />
        </Fragment>
      );
    };
    
  
        "#;

    let fm = cm.new_source_file(FileName::Custom("test.js".into()), src.into());

    let lexer = Lexer::new(
        // We want to parse ecmascript
        Syntax::Typescript(TsConfig {
            tsx: true,
            decorators: true,
            dts: true,
            no_early_errors: true,
        }),
        // EsVersion defaults to es5
        Default::default(),
        StringInput::from(&*fm),
        None,
    );

    let mut parser = Parser::new_from(lexer);

    let module = parser.parse_module();

    if let Ok(module) = module {
        let module_item = module.body;
        // print!("{:?}", module_item);

        for item in module_item {
            match item {
                ModuleItem::ModuleDecl(_) => (),
                ModuleItem::Stmt(stmt) => collect_from_jsx_nodes::<Header>(stmt),
            }
        }
    }
}
