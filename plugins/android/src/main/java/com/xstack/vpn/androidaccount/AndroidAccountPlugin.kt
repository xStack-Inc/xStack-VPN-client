package com.xstack.vpn.androidaccount

import android.accounts.AccountManager
import android.app.Activity
import android.content.ActivityNotFoundException
import android.content.Context
import android.content.Intent
import android.os.Build
import androidx.activity.result.ActivityResult
import app.tauri.annotation.ActivityCallback
import app.tauri.annotation.Command
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin

@TauriPlugin
class AndroidAccountPlugin(private val activity: Activity): Plugin(activity) {
    companion object {
        private const val PREFS_NAME = "xstack_vpn_android_account"
        private const val KEY_ACCOUNT_NAME = "account_name"
        private const val KEY_ACCOUNT_TYPE = "account_type"
    }
    @Command
    fun requestAccount(invoke: Invoke) {
        val saved = savedAccount()
        if (saved != null) {
            resolveGranted(invoke, saved.name, saved.type)
            return
        }

        val intent = chooseAccountIntent()

        try {
            startActivityForResult(invoke, intent, "accountPickerResult")
        } catch (_: ActivityNotFoundException) {
            resolveDenied(invoke, "account_picker_unavailable")
        }
    }

    @ActivityCallback
    fun accountPickerResult(invoke: Invoke, result: ActivityResult) {
        if (result.resultCode != Activity.RESULT_OK) {
            resolveDenied(invoke, "cancelled")
            return
        }

        val data = result.data
        val accountName = data?.getStringExtra(AccountManager.KEY_ACCOUNT_NAME)
        val accountType = data?.getStringExtra(AccountManager.KEY_ACCOUNT_TYPE)

        if (accountName.isNullOrBlank()) {
            resolveDenied(invoke, "empty_account")
            return
        }

        saveAccount(accountName, accountType)
        resolveGranted(invoke, accountName, accountType)
    }

    private fun chooseAccountIntent(): Intent {
        val description = "Выберите рабочий аккаунт для xStack VPN"

        return if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.M) {
            AccountManager.newChooseAccountIntent(
                null,
                null,
                null,
                description,
                null,
                null,
                null,
            )
        } else {
            @Suppress("DEPRECATION")
            AccountManager.newChooseAccountIntent(
                null,
                null,
                null,
                true,
                description,
                null,
                null,
                null,
            )
        }
    }

    private data class StoredAccount(val name: String, val type: String?)

    private fun savedAccount(): StoredAccount? {
        val prefs = activity.getSharedPreferences(PREFS_NAME, Context.MODE_PRIVATE)
        val name = prefs.getString(KEY_ACCOUNT_NAME, null)?.takeIf { it.isNotBlank() }
            ?: return null
        val type = prefs.getString(KEY_ACCOUNT_TYPE, null)?.takeIf { it.isNotBlank() }
        return StoredAccount(name, type)
    }

    private fun saveAccount(name: String, type: String?) {
        activity.getSharedPreferences(PREFS_NAME, Context.MODE_PRIVATE)
            .edit()
            .putString(KEY_ACCOUNT_NAME, name)
            .putString(KEY_ACCOUNT_TYPE, type)
            .apply()
    }

    private fun resolveGranted(invoke: Invoke, name: String, type: String?) {
        val response = JSObject()
        response.put("granted", true)
        response.put("email", name)
        response.put("accountType", type)
        response.put("reason", null)
        invoke.resolve(response)
    }

    private fun resolveDenied(invoke: Invoke, reason: String) {
        val response = JSObject()
        response.put("granted", false)
        response.put("email", null)
        response.put("accountType", null)
        response.put("reason", reason)
        invoke.resolve(response)
    }
}
