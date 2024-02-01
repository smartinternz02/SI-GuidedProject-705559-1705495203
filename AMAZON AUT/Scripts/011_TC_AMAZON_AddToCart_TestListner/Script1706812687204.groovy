import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

//WebUI.openBrowser('')
//
//WebUI.navigateToUrl('https://www.amazon.com/')
//
//WebUI.click(findTestObject('Object Repository/AMAZON_ValidateAddToCart_OR/Page_Amazon.com. Spend less. Smile more/span_Hello, sign in'))
//
//WebUI.setText(findTestObject('Object Repository/AMAZON_ValidateAddToCart_OR/Page_Amazon Sign-In/input_email'), '9317171704')
//
//WebUI.click(findTestObject('Object Repository/AMAZON_ValidateAddToCart_OR/Page_Amazon Sign-In/inputcontinue'))
//
//WebUI.setEncryptedText(findTestObject('Object Repository/AMAZON_ValidateAddToCart_OR/Page_Amazon Sign-In/input_password'), 
//    'VOnTHcYiLTbiLaiba9HOxQ==')
//
//WebUI.click(findTestObject('Object Repository/AMAZON_ValidateAddToCart_OR/Page_Amazon Sign-In/inputsignInSubmit'))

WebUI.setText(findTestObject('Object Repository/AMAZON_ValidateAddToCart_OR/Page_Amazon.com. Spend less. Smile more/input_field-keywords'), 
    'ikigai book')

WebUI.click(findTestObject('Object Repository/AMAZON_ValidateAddToCart_OR/Page_Amazon.com. Spend less. Smile more/inputnav-search-submit-button'))

WebUI.click(findTestObject('Object Repository/AMAZON_ValidateAddToCart_OR/Page_Amazon.com  ikigai book/span_Ikigai The Japanese Secret to a Long a_7b4a50'))

WebUI.click(findTestObject('Object Repository/AMAZON_ValidateAddToCart_OR/Page_Amazon.com Ikigai The Japanese Secret _084d27/input_submit.add-to-cart'))

//WebUI.closeBrowser()

