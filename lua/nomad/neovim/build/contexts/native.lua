---@type nomad.neovim.build.ContextOpts
return {
  block_on_build = function(build_fut, error_lvl)
    --- We need to manually schedule the event loop to keep polling the future
    --- until it completes. Unlike lazy.nvim which handles coroutine scheduling,
    --- we use vim.schedule() to yield control back to Neovim's event loop.
    ---
    ---@type nomad.future.Context
    local ctx = {
      wake = function()
        -- Schedule a check in the next event loop tick
        vim.schedule(function() end)
      end,
    }

    ---@type nomad.Result<nil, string>
    local build_res

    -- Keep polling the future until it completes.
    while true do
      local maybe_res = build_fut.poll(ctx)

      if maybe_res:is_some() then
        build_res = maybe_res:unwrap()
        break
      end

      vim.cmd 'redraw'
    end

    if build_res:is_err() then
      error(build_res:unwrap_err(), error_lvl)
    end
  end,

  notify = function(message)
    vim.print(message)
  end,
}
