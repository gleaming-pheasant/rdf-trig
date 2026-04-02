#[macro_export]
macro_rules! impl_write_sync_and_async {
    (
        $target:ty, $this:ident,
        $trait_sync:ident, $method_sync:ident,
        $trait_async:ident, $method_async:ident,
        [ $($part:expr),* $(,)? ]
    ) => {
        impl $trait_sync for $target {
            fn $method_sync<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
                let $this = self;
                $(
                    writer.write_all($part)?;
                )*
                Ok(())
            }
        }

        #[cfg(feature = "tokio")]
        impl $trait_async for $target {
            async fn $method_async<W>(&self, writer: &mut W) -> std::io::Result<()>
            where
                W: tokio::io::AsyncWrite + Unpin + Send,
            {
                use tokio::io::AsyncWriteExt;
                let $this = self;
                $(
                    writer.write_all($part).await?;
                )*
                Ok(())
            }
        }
    };
}